# MultiPolygon

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**type_** | **String** |  | 
**bbox** | **Vec<f64>** | This parameter contains information on the coordinate range for its Geometries, Features or FeatureCollections. It describes a bounding box for this. The coordinates are given in the decimal degrees unit. In the following it is described, which value of the array represents which coordinate:   - The most southwesterly point: Longitude   - The most southwesterly point: Latitude   - The most northeasterly point: Longitude   - The most northeasterly point: Latitude  | [optional] [default to None]
**coordinates** | [**Vec<Vec<Vec<Vec<f64>>>>**](array.md) | A Polygon is a surface specified by its vertices. The vertices are coordinates, given in the decimal degrees unit. Note the following:   - The surface vertices are given counterclockwise.   - Holes in the surface are given clockwise.   - The last and the first coordinates must be equal. In the following it is described, which value of the innermost array represents which coordinate:   - Longitude   - Latitude The outermost array includes the various Polygons. A example, given in the [RFC7946 - The GeoJSON format](https://tools.ietf.org/html/rfc7946#appendix-A.6)  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


