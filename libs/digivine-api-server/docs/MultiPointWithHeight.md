# MultiPointWithHeight

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**type_** | **String** |  | 
**bbox** | **Vec<f64>** | This parameter contains information on the coordinate range for its Geometries, Features or FeatureCollections. It describes a bounding box for this. The coordinates are given in the decimal degrees unit. The altitude is given in meter. In the following it is described, which value of the array represents which coordinate:   - The most southwesterly point: Longitude   - The most southwesterly point: Latitude   - The most southwesterly point: Altitude   - The most northeasterly point: Longitude   - The most northeasterly point: Latitude   - The most northeasterly point: Altitude  | [optional] [default to None]
**coordinates** | [**Vec<Vec<f64>>**](array.md) | An array of Points with its coordinates and altitudes, given in the decimal degrees unit. The altitude/height is given in meter. In the following it is described, which value of the array represents which coordinate:   - Longitude   - Latitude   - Altitude  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


