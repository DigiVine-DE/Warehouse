# ForecastMeasurement

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | The numeric ID of the Measurement | [optional] [default to None]
**time** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Time of measurement, given in the format YYYY-MM-DDThh:mm:ssZ (where Y,M,D,h,m,s are variables for the time). | 
**values** | [***serde_json::Value**](.md) | An object containing the values of the measurement. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


