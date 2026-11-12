from django.urls import path
from . import views


urlpatterns = [
    path('get/<int:object_key>/<hash_key>/', views.Plant.plant_get, name='plant_get'),
    path('post/<int:object_key>/<hash_key>/', views.Plant.plant_post, name='plant_post'),
    path('send/<int:object_key>/<hash_key>/', views.Plant.plant_send, name='plant_send'),
]
