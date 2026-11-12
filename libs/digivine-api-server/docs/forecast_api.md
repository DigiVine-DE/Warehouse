# forecast_api

All URIs are relative to *http://127.0.0.1/api/v0.3*

Method | HTTP request | Description
------------- | ------------- | -------------
**deleteForecast**](forecast_api.md#deleteForecast) | **DELETE** /forecast | Can't delete complete collection
**deleteOneForecast**](forecast_api.md#deleteOneForecast) | **DELETE** /forecast/{ForecastId} | Removes a specific stored Forecast from the collection
**getAllForecasts**](forecast_api.md#getAllForecasts) | **GET** /forecast | Returns a list of (all) stored Forecasts
**getOneForecast**](forecast_api.md#getOneForecast) | **GET** /forecast/{ForecastId} | Returns a specific stored Forecast from the collection
**postForecast**](forecast_api.md#postForecast) | **POST** /forecast | Adds a new Forecast to the collection


# **deleteForecast**
> deleteForecast(ctx, ctx, )
Can't delete complete collection

You can't delete the complete collection of Forecast data. You can delete one specific Forecast with `DELETE /forecast/{ForecastID}` (`deleteOneForecast`). 

### Required Parameters
This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteOneForecast**
> deleteOneForecast(ctx, ctx, forecast_id)
Removes a specific stored Forecast from the collection

Removes a specific stored Forecast from the collection. The parameter `ForecastId` is required, to delete the specific stored Forecast. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **forecast_id** | **i64**| The numeric ID of the forecast | 

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllForecasts**
> Vec<models::Forecast> getAllForecasts(ctx, ctx, optional)
Returns a list of (all) stored Forecasts

Returns a list of (all) stored Forecasts. With the parameters `Limit` and `Offset` it can be specified, which data to return. If the operation was successful (status code `200`), an array with Forecasts will be returned. 

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
 **offset** | **i32**| The number of items to skip before starting to collect the result set | [default to 0]
 **limit** | **i32**| The number of items to return | [default to 100]

### Return type

[**Vec<models::Forecast>**](Forecast.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOneForecast**
> models::Forecast getOneForecast(ctx, ctx, forecast_id)
Returns a specific stored Forecast from the collection

Returns a specific stored Forecast from the collection. The parameter `ForecastId` is required, to get the specific stored Forecast. If the operation was successful (status code `200`), the specific Forecast will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **forecast_id** | **i64**| The numeric ID of the forecast | 

### Return type

[**models::Forecast**](Forecast.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postForecast**
> models::InlineResponse201 postForecast(ctx, ctx, forecast)
Adds a new Forecast to the collection

Adds a new Forecast to the collection. In the request body you have to add the new Forecast, which to be added. If the operation was successful (status code `200`), the numeric ID of the new Forecast will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **forecast** | [**Forecast**](Forecast.md)| A Forecast to be added to the collection | 

### Return type

[**models::InlineResponse201**](inline_response_201.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

