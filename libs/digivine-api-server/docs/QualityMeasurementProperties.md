# QualityMeasurementProperties

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | The numeric ID of the Quality Measurement | [optional] [default to None]
**plant_id** | **i64** | The numeric ID of the Plant | 
**timestamp** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Timestamp when the Measurement was taken, given in the format YYYY-MM-DDThh:mm:ss.sssZ (where Y,M,D,h,m,s are variables for the time). | 
**device** | **String** | Device id, needed for e.g. assigning the device to a customer/user | 
**measurement_uuid** | [***uuid::Uuid**](UUID.md) | A Universally Unique IDentifier (UUID) of the Measurement | 
**tartaric_acid** | **f64** | The tartaric acid concentration, measured in g/L | [optional] [default to None]
**malic_acid** | **f64** | The malic acid concentration, measured in g/L | 
**fructose** | **f64** | The fructose concentration, measured in g/L | 
**glucose** | **f64** | The glucose concentration, measured in g/L | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


