from django.urls import path
from . import views

urlpatterns = [
    path('convert_geojson_to_taskdata/<hash_key>/', views.Spray.spray_convert_geojson_to_taskdata
         , name='spray_convert_geojson_to_taskdata'),
    path('convert_taskdata_to_geojson/<hash_key>/', views.Spray.spray_convert_taskdata_to_geojson
         , name='spray_convert_taskdata_to_geojson')
]
