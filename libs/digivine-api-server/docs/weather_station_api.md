# weather_station_api

All URIs are relative to *http://127.0.0.1/api/v0.3*

Method | HTTP request | Description
------------- | ------------- | -------------
**deleteOneWeatherStation**](weather_station_api.md#deleteOneWeatherStation) | **DELETE** /weather/station/{WeatherStationId} | Removes a specific weather station from the collection
**getAllWeatherStations**](weather_station_api.md#getAllWeatherStations) | **GET** /weather/station | Returns a list of all weather stations
**getOneWeatherStation**](weather_station_api.md#getOneWeatherStation) | **GET** /weather/station/{WeatherStationId} | Returns a specific weather station from the collection
**patchOneWeatherStation**](weather_station_api.md#patchOneWeatherStation) | **PATCH** /weather/station/{WeatherStationId} | Patches a specific weather station
**postWeatherStation**](weather_station_api.md#postWeatherStation) | **POST** /weather/station | Adds a new Weather Station to the collection


# **deleteOneWeatherStation**
> deleteOneWeatherStation(ctx, ctx, weather_station_id)
Removes a specific weather station from the collection

Removes a specific weather station from the collection. The parameter `station_id` is required to delete the specific weather station. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **weather_station_id** | **i32**| The numeric ID of the weather station | 

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllWeatherStations**
> Vec<models::WeatherStation> getAllWeatherStations(ctx, ctx, optional)
Returns a list of all weather stations

Returns a list of all weather stations. With the optional parameter `name`, only the weather stations with names containing all specified keywords (case insensitive) will be returned. With the optional parameter `bundesland`, only the weather stations located in the specified state will be returned. With the optional parameter `box2d`, only the weather stations that intersect the given bounding box will be returned. With the optional parameters `offset` and `limit`, you can control the pagination of the results. If the operation was successful (status code `200`), an array with weather stations will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **bundesland** | [**String**](String.md)| A set of keywords used to filter the results of weather stations. Only weather stations located in the specified state containing all of the specified keywords (case insensitive) will be returned. | 
 **name** | [**String**](String.md)| A set of keywords used to filter the results of weather stations. Only weather stations with names containing all specified keywords (case insensitive) will be returned. | 
 **box2d** | [**f64**](f64.md)| The two-dimensional geometry bounding box | 
 **offset** | **i32**| The number of items to skip before starting to collect the result set | [default to 0]
 **limit** | **i32**| The number of items to return | [default to 100]

### Return type

[**Vec<models::WeatherStation>**](WeatherStation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOneWeatherStation**
> models::WeatherStation getOneWeatherStation(ctx, ctx, weather_station_id)
Returns a specific weather station from the collection

Returns a specific weather station from the collection. The parameter `station_id` is required to get the specific weather station. If the operation was successful (status code `200`), the specific weather station will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **weather_station_id** | **i32**| The numeric ID of the weather station | 

### Return type

[**models::WeatherStation**](WeatherStation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patchOneWeatherStation**
> models::WeatherStation patchOneWeatherStation(ctx, ctx, weather_station_id, weather_station_patch)
Patches a specific weather station

Patches a specific weather station. The parameter `station_id` is required to replace the specific weather station with the new one. Also, the new data (`WeatherStationPatch`) is required. If the operation was successful (status code `200`), the new weather station will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **weather_station_id** | **i32**| The numeric ID of the weather station | 
  **weather_station_patch** | [**WeatherStationPatch**](WeatherStationPatch.md)| The updated properties to patch | 

### Return type

[**models::WeatherStation**](WeatherStation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postWeatherStation**
> models::InlineResponse2018 postWeatherStation(ctx, ctx, weather_station)
Adds a new Weather Station to the collection

Adds a new Weather Station to the collection. If the operation was successful (status code `200`), the numeric ID of the new Weather Station will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **weather_station** | [**WeatherStation**](WeatherStation.md)| A Weather Station to be added to the collection | 

### Return type

[**models::InlineResponse2018**](inline_response_201_8.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

