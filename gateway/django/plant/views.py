import csv
import logging
import requests
import json

from django.http import HttpResponse
from django.db import connection, transaction
from django.views.decorators.csrf import csrf_exempt
from rest_framework.decorators import api_view

from .forms import UploadFileForm
from rest_framework.views import APIView

# Create your views here.
from .models import Plant

'''
    https://stackoverflow.com/questions/5931586/raw-sql-queries-in-django-views
    https://docs.djangoproject.com/en/3.1/topics/http/file-uploads/#
    https://blog.etianen.com/blog/2013/10/05/python-unicode-streams/
    https://hackernoon.com/automatically-set-csrf-token-in-postman-django-tips-c9ec8eb9eb5b
    https://stackabuse.com/python-string-interpolation-with-the-percent-operator/
    https://stackoverflow.com/questions/11322430/how-to-send-post-request
    https://stackoverflow.com/questions/9733638/how-to-post-json-data-with-python-requests
'''
logger = logging.getLogger('django')


class Plant(APIView):

    @staticmethod
    @api_view(['GET'])
    def plant_get(self, object_key, hash_key):
        if hash_key == '8gf5tzvhk12XYZquioTTmn365':
            result = Plant.handle_get_geojson_for_planter_by_object_key(self, object_key)
            antwort = HttpResponse(json.dumps(result))
            antwort['Content-Type'] = 'application/json'
            return antwort
        return HttpResponse(status=201)

    @csrf_exempt
    @api_view(['POST'])
    def plant_post(self, object_key, hash_key):
        logger.info('plant_post')
        if hash_key == '8gf5tzvhk12XYZquioTTmn365' and self.method == 'POST':
            form = UploadFileForm(self.POST, self.FILES)
            if form.is_valid():
                Plant.handle_uploaded_file(self, self.FILES['file'], object_key)
                return HttpResponse(status=201)
        return HttpResponse(status=201)

    @staticmethod
    @api_view(['GET'])
    def plant_send(self, object_key, hash_key):
        logger.info('plant_send')
        if hash_key == '8gf5tzvhk12XYZquioTTmn365':
            base_url = Plant.handle_get_system_parameter(self, 'dataWareHouseServerUrl')[0]
            field_id = Plant.handle_get_field_id(self, object_key)[0]
            result_as_json = Plant.handle_get_geojson_for_planter_by_object_key(self, object_key)[0]

            final_url = str(base_url) + '/api/v0.3/field/' + str(field_id) + '/plant?api-token=all'
            header = {'Content-type': 'application/json', 'Accept': 'application/json'}

            response_decoded_json = requests.post(final_url, json=result_as_json, headers=header)
            response_json = response_decoded_json.json()

            external_id = response_json["id"]
            Plant.handle_update_external_id_for_object_key(self, external_id, object_key)
            return HttpResponse(status=201)
        return HttpResponse(status=201)

    @staticmethod
    def handle_update_external_id_for_object_key(self, external_id, object_key):
        cursor = connection.cursor()
        cursor.execute("update plant set external_id = %s where import_key = %s"
                       , [external_id, "ta_" + str(object_key)])

    @staticmethod
    def handle_get_system_parameter(self, key):
        cursor = connection.cursor()
        cursor.execute("select wert from systemparameter where schluessel = %s", [key])
        result = cursor.fetchone()
        return result

    @staticmethod
    def handle_get_field_id(self, object_key):
        cursor = connection.cursor()
        cursor.execute("select distinct field_id from plant where import_key = %s", ["ta_" + str(object_key)])
        result = cursor.fetchone()
        return result

    @staticmethod
    def handle_get_geojson_for_planter_by_object_key(self, object_key):
        cursor = connection.cursor()
        cursor.execute("select planter_to_geojson(%s)", ["ta_" + str(object_key)])
        result = cursor.fetchone()
        return result

    @staticmethod
    def handle_uploaded_file(self, f, object_key):
        logger.info('handle_uploaded_file')
        with transaction.atomic():
            with connection.cursor() as cursor:
                cursor.execute("delete from plant where import_key = %s", ["ta_" + str(object_key)])
                csv_file = f.read().decode('utf-16LE').replace(u'\ufeff', u'').replace('  ', ';').splitlines()
                csv_reader = csv.reader(csv_file, delimiter=';')
                for dataRow in csv_reader:
                    sql = "insert into plant(import_key, timestamp, latitude, longitude, height, deviation"
                    sql += ", speed, plant_distance, row_distance, row_number, geom_utm32, user_id, terminal_id"
                    sql += ", field_id) values ("
                    sql += "'ta_" + str(object_key) + "', "
                    sql += "to_timestamp('" + dataRow[0] + dataRow[1] + "', 'DD.MM.YYYYHH:MI:SS.MS'), "
                    sql += dataRow[2] + ", "
                    sql += dataRow[3] + ", "
                    sql += dataRow[10] + ", "
                    sql += dataRow[13] + ", "
                    sql += dataRow[15] + ", "
                    sql += dataRow[21] + ", "
                    sql += dataRow[22] + ", "
                    sql += dataRow[31] + ", "
                    sql += "ST_Transform(ST_SetSRID(ST_MakePoint(" + dataRow[3] + ", " + dataRow[2] + "), 4326), 25832) "
                    sql += ", 1, 1, 1)"
                    cursor.execute(sql)