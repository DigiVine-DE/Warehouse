import logging
import re
import requests

from io import BytesIO
from requests.structures import CaseInsensitiveDict
from django.http import HttpResponse
from django.views.decorators.csrf import csrf_exempt
from rest_framework.decorators import api_view
from rest_framework.views import APIView

logger = logging.getLogger('django')

'''
https://www.django-rest-framework.org/api-guide/views/
'''

class Spray(APIView):

    @csrf_exempt
    @api_view(['POST'])
    def spray_convert_geojson_to_taskdata(self, hash_key):
        url = "http://127.0.0.1:9080/isoxmlwebservice/command/task/spray/convert_geojson_to_taskdata?hashkey=" + hash_key
        return Spray.redirect(self, url)

    @csrf_exempt
    @api_view(['POST'])
    def spray_convert_taskdata_to_geojson(self, hash_key):
        url = "http://127.0.0.1:9080/isoxmlwebservice/command/task/spray/convert_taskdata_to_geojson?hashkey=" + hash_key
        return Spray.redirect(self, url)

    def redirect(self, url):
        requestBodyBinary = BytesIO(self.body)
        requestHeadersContentType = self.headers["Content-Type"]
        requestHeadersFilename = re.findall("filename=(.+)", self.headers["Content-Disposition"])[0]

        headers = CaseInsensitiveDict()
        headers["Content-Type"] = requestHeadersContentType
        headers["Content-Disposition"] = "attachment; filename=" + requestHeadersFilename
        responseFromIsoXMLService = requests.post(url, headers=headers, data=requestBodyBinary.getvalue())

        responseFromIsoXMLServiceHeadersContentType = responseFromIsoXMLService.headers["Content-Type"]
        responseFromIsoXMLServiceHeadersFilename = \
        re.findall("filename=(.+)", responseFromIsoXMLService.headers["Content-Disposition"])[0]
        responseFromIsoXMLServiceContentBinary = BytesIO(responseFromIsoXMLService.content)

        response = HttpResponse(responseFromIsoXMLServiceContentBinary.getvalue())
        response["Content-Type"] = responseFromIsoXMLServiceHeadersContentType
        response["Content-Disposition"] = "attachment; filename=" + responseFromIsoXMLServiceHeadersFilename

        return response
