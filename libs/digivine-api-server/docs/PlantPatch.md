# PlantPatch

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**import_key** | **String** | Internal key to facilitate error tracing | [optional] [default to None]
**timestamp** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Timestamp when this plant was planted, given in the format YYYY-MM-DDThh:mm:ss.sssZ (where Y,M,D,h,m,s are variables for the time). | [optional] [default to None]
**height** | **f64** | Height above sea level, measured in millimeters | [optional] [default to None]
**deviation** | **f64** |  | [optional] [default to None]
**speed** | **f64** | Planting speed measured in km/h | [optional] [default to None]
**plant_distance** | **f64** | Distance between two Plants in a row, measured in millimeters | [optional] [default to None]
**row_distance** | **f64** | Distance between two rows, measured in millimeters | [optional] [default to None]
**row_number** | **i32** | Number of the row of the Plant | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


