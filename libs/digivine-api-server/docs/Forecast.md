# Forecast

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | The numeric ID of the Forecast | [optional] [default to None]
**group_id** | **i64** | Notice. This information will soon no longer be used and sent along! | 
**station_id** | **i64** | The numeric ID of the station that makes Forecasts for a group of Fields. | 
**station** | **String** | Name of the measuring station | 
**field_ids** | **Vec<i64>** | An array containing the IDs of the Fields for which the Forecast is. | 
**model_id** | **i64** | The numeric ID of the model | 
**model** | **String** | Name of the model | 
**query_timestamp** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Time of retrieval of the data from an external source, given in the format YYYY-MM-DDThh:mm:ss.sssZ (where Y,M,D,h,m,s are variables for the time). | 
**time_span_start** | [***chrono::DateTime::<chrono::Utc>**](date.md) | Start time of the measurement, given in the format YYYY-MM-DDThh:mm:ssZ (where Y,M,D,h,m,s are variables for the time). | 
**time_span_end** | [***chrono::DateTime::<chrono::Utc>**](date.md) | End time of the measurement, given in the format YYYY-MM-DDThh:mm:ssZ (where Y,M,D,h,m,s are variables for the time). | 
**measurement_rate** | **String** | Rate of the measurement | 
**measurements** | [**Vec<models::ForecastMeasurement>**](ForecastMeasurement.md) | An array of measurements | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


