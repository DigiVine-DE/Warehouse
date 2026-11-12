# weather_data_api

All URIs are relative to *http://127.0.0.1/api/v0.3*

Method | HTTP request | Description
------------- | ------------- | -------------
**deleteOneWeatherData**](weather_data_api.md#deleteOneWeatherData) | **DELETE** /weather/data/{WeatherStationId}/{MeasurementDate} | Removes weather data for a specific station and date
**getAllWeatherData**](weather_data_api.md#getAllWeatherData) | **GET** /weather/data | Returns a list of weather data
**getOneWeatherData**](weather_data_api.md#getOneWeatherData) | **GET** /weather/data/{WeatherStationId}/{MeasurementDate} | Returns weather data for a specific station and date
**patchOneWeatherData**](weather_data_api.md#patchOneWeatherData) | **PATCH** /weather/data/{WeatherStationId}/{MeasurementDate} | Patches weather data for a specific station and date
**postWeatherData**](weather_data_api.md#postWeatherData) | **POST** /weather/data | Adds new weather data


# **deleteOneWeatherData**
> deleteOneWeatherData(ctx, ctx, weather_station_id, measurement_date)
Removes weather data for a specific station and date

Removes weather data for a specific weather station and date. The parameters `WeatherStationId` and `MeasurementDate` are required to delete the specific weather data. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **weather_station_id** | **i32**| The numeric ID of the weather station | 
  **measurement_date** | **String**| The reference date for the weather data | 

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllWeatherData**
> Vec<models::WeatherData> getAllWeatherData(ctx, ctx, optional)
Returns a list of weather data

Returns a list of weather data sorted by `measurement_date`. With the optional parameter `stationIds`, only the weather data for the specified station ids will be returned. With the optional parameters `startDate` and `endDate`, only the weather data within the specified date range (inclusive) will be returned. If `endDate` is omitted, it defaults to today's date. If `startDate` is omitted, it includes all data up to `endDate`. If the operation was successful (status code `200`), an array with weather data will be returned. 

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
 **station_ids** | [**i32**](i32.md)| A set of station ids used to filter the results of weather data. Only weather data for the specified station ids will be returned. | 
 **start_date** | **String**| The start date of the date range used to filter the results of weather data. Only weather data within the specified date range (inclusive) will be returned. If omitted, it includes all data up to `endDate`. | 
 **end_date** | **String**| The end date of the date range used to filter the results of weather data. Only weather data within the specified date range (inclusive) will be returned. If omitted, it defaults to today's date. | 
 **offset** | **i32**| The number of items to skip before starting to collect the result set | [default to 0]
 **limit** | **i32**| The number of items to return | [default to 100]

### Return type

[**Vec<models::WeatherData>**](WeatherData.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOneWeatherData**
> models::WeatherData getOneWeatherData(ctx, ctx, weather_station_id, measurement_date)
Returns weather data for a specific station and date

Returns weather data for a specific weather station and date. The parameters `WeatherStationId` and `MeasurementDate` are required to get the specific weather data. If the operation was successful (status code `200`), the weather data will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **weather_station_id** | **i32**| The numeric ID of the weather station | 
  **measurement_date** | **String**| The reference date for the weather data | 

### Return type

[**models::WeatherData**](WeatherData.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patchOneWeatherData**
> models::WeatherData patchOneWeatherData(ctx, ctx, weather_station_id, measurement_date, weather_data_patch)
Patches weather data for a specific station and date

Patches weather data for a specific weather station and date. The parameters `WeatherStationId` and `MeasurementDate` are required to replace the specific weather data with the new one. Also, the new data (`WeatherDataPatch`) is required. If the operation was successful (status code `200`), the new weather data will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **weather_station_id** | **i32**| The numeric ID of the weather station | 
  **measurement_date** | **String**| The reference date for the weather data | 
  **weather_data_patch** | [**WeatherDataPatch**](WeatherDataPatch.md)| The updated properties to patch | 

### Return type

[**models::WeatherData**](WeatherData.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postWeatherData**
> models::InlineResponse2019 postWeatherData(ctx, ctx, weather_data)
Adds new weather data

Adds new weather data to the collection. If the operation was successful (status code `201`), the ID of the new weather data entry will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **weather_data** | [**WeatherData**](WeatherData.md)| Weather data to be added to the collection | 

### Return type

[**models::InlineResponse2019**](inline_response_201_9.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

