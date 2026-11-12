# cultivar_api

All URIs are relative to *http://127.0.0.1/api/v0.3*

Method | HTTP request | Description
------------- | ------------- | -------------
**deleteOneCultivar**](cultivar_api.md#deleteOneCultivar) | **DELETE** /vivc/{CultivarId} | Removes a specific cultivar from the collection bi its ID
**getAllCultivars**](cultivar_api.md#getAllCultivars) | **GET** /vivc | Returns a list of all stored cultivars
**getOneCultivar**](cultivar_api.md#getOneCultivar) | **GET** /vivc/{CultivarId} | Returns a specific cultivar from the collection by its ID
**patchOneCultivar**](cultivar_api.md#patchOneCultivar) | **PATCH** /vivc/{CultivarId} | Patches a specific cultivar
**postCultivar**](cultivar_api.md#postCultivar) | **POST** /vivc | Adds a new Cultivar to the collection


# **deleteOneCultivar**
> deleteOneCultivar(ctx, ctx, cultivar_id)
Removes a specific cultivar from the collection bi its ID

Removes a specific cultivar from the collection. The parameter `CultivarId` is required to delete the specific cultivar. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cultivar_id** | **i32**| The numeric ID of the Cultivar | 

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllCultivars**
> Vec<models::Cultivar> getAllCultivars(ctx, ctx, optional)
Returns a list of all stored cultivars

Returns a list of all stored cultivars. With the optional parameter `color`, only the cultivars with the specified berry skin color will be returned. With the optional parameter `country`, only the cultivars originating from the specified country or region will be returned. With the optional parameter `species`, only the cultivars belonging to the specified species will be returned. With the optional parameter `primeName`, only the cultivars with the specified prime name will be returned. With the optional parameter `cultivarName`, only the cultivars with the specified cultivar name will be returned. If the operation was successful (status code `200`), an array with cultivars will be returned. 

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
 **color** | [**String**](String.md)| A set of keywords used to filter the results of cultivars. Only cultivars with a berry skin color containing all of the specified keywords (case insensitive) will be returned. | 
 **region** | [**String**](String.md)| A set of keywords used to filter the results of cultivars. Only cultivars with a country or region containing all of the specified keywords (case insensitive) will be returned. | 
 **species** | [**String**](String.md)| A set of keywords used to filter the results of cultivars. Only cultivars with a species containing all of the specified keywords (case insensitive) will be returned. | 
 **prime_name** | [**String**](String.md)| A set of keywords used to filter the results of cultivars. Only cultivars with prime names containing all specified keywords (case insensitive) will be returned. | 
 **cultivar_name** | [**String**](String.md)| A set of keywords used to filter the results of cultivars. only cultivars with names containing all specified keywords (case insensitive) will be returned. | 
 **offset** | **i32**| The number of items to skip before starting to collect the result set | [default to 0]
 **limit** | **i32**| The number of items to return | [default to 100]

### Return type

[**Vec<models::Cultivar>**](Cultivar.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOneCultivar**
> models::Cultivar getOneCultivar(ctx, ctx, cultivar_id)
Returns a specific cultivar from the collection by its ID

Returns a specific cultivar from the collection. The parameter `CultivarId` is required to get the specific cultivar. If the operation is successful (status code `200`), the specific cultivar will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cultivar_id** | **i32**| The numeric ID of the Cultivar | 

### Return type

[**models::Cultivar**](Cultivar.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patchOneCultivar**
> models::Cultivar patchOneCultivar(ctx, ctx, cultivar_id, cultivar_patch)
Patches a specific cultivar

Patches a specific cultivar. The parameter `CultivarId` is required to replace the specific cultivar with the new one. Also, the new data (`CultivarPatch`) is required. If the operation is successful (status code `200`), the new cultivar will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cultivar_id** | **i32**| The numeric ID of the Cultivar | 
  **cultivar_patch** | [**CultivarPatch**](CultivarPatch.md)| The updated properties to patch | 

### Return type

[**models::Cultivar**](Cultivar.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postCultivar**
> models::InlineResponse2017 postCultivar(ctx, ctx, cultivar)
Adds a new Cultivar to the collection

Adds a new Cultivar to the collection. If the operation was successful (status code `200`), the numeric ID of the new Cultivar will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cultivar** | [**Cultivar**](Cultivar.md)| A Cultivar to be added to the collection | 

### Return type

[**models::InlineResponse2017**](inline_response_201_7.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

