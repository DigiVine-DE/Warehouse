# PlantProperties

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | The numeric ID of the Plant | [optional] [default to None]
**import_key** | **String** | Internal key to facilitate error tracing | 
**timestamp** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Timestamp when this plant was planted, given in the format YYYY-MM-DDThh:mm:ss.sssZ (where Y,M,D,h,m,s are variables for the time). | 
**height** | **f64** | Height above sea level, measured in millimeters | 
**deviation** | **f64** |  | 
**speed** | **f64** | Planting speed measured in km/h | 
**plant_distance** | **f64** | Distance between two Plants in a row, measured in millimeters | 
**row_distance** | **f64** | Distance between two rows, measured in millimeters | 
**row_number** | **i32** | Number of the row of the Plant | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


