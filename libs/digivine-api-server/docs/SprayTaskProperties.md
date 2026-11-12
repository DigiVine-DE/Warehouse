# SprayTaskProperties

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**task_id** | **i64** | The numeric ID of the Harvest Task | 
**task_name** | **String** | Name of the task | 
**customer_name** | **String** | Name of the customer | 
**farm_name** | **String** | Name of the farm | [optional] [default to None]
**fruit_group_name** | **String** | Name of the fruit group | 
**fruit_name** | **String** | Name of the specific fruit | 
**plain_name** | **String** | Name of the plains | 
**plain_size** | **i64** | Size of the plains in square meter | 
**application_total_volume** | **i64** | The total volume of protection sprayed | [optional] [default to None]
**total_area** | **i64** | The total area of the spray task in square meter | [optional] [default to None]
**effective_total_distance** | **i64** | The effective distance travelled by the spray machine in meter | [optional] [default to None]
**effective_total_time** | **i64** | The effective total time of the spray run in minutes | [optional] [default to None]
**plant_protection** | [**Vec<models::SprayTaskPropertiesPlantProtection>**](SprayTask_properties_plant_protection.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


