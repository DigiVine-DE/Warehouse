# from django.db import models
from django.contrib.gis.db import models

# Create your models here.
# https://docs.djangoproject.com/en/3.1/ref/models/
# https://docs.djangoproject.com/en/3.1/ref/models/fields/
# https://docs.djangoproject.com/en/3.1/ref/contrib/gis/model-api/

# TODO: ...


class Plant(models.Model):
    id = models.BigAutoField(primary_key=True)
    import_key = models.CharField(max_length=1024, null=False)
    timestamp = models.DateTimeField(null=False)
    latitude = models.FloatField(null=False)
    longitude = models.FloatField(null=False)
    height = models.FloatField(null=False)
    deviation = models.FloatField(null=False)
    speed = models.FloatField(null=False)
    plant_distance = models.FloatField(null=False)
    row_distance = models.FloatField(null=False)
    row_number = models.IntegerField(null=False)
    user_id = models.BigIntegerField(null=False)
    terminal_id = models.BigIntegerField(null=False)
    field_id = models.BigIntegerField(null=False)
    external_id = models.BigIntegerField(null=True)
    geom_utm32 = models.PointField(srid=25832, null=True)

    class Meta:
        db_table = 'plant'

    def __str__(self):
        return self.import_key
