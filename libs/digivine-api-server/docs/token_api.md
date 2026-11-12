# token_api

All URIs are relative to *http://127.0.0.1/api/v0.3*

Method | HTTP request | Description
------------- | ------------- | -------------
**deleteOneAccessRight**](token_api.md#deleteOneAccessRight) | **DELETE** /token/{UserId}/{Tokenstr}/access_right/{AccessRightPath} | Deletes a specific access right for a specific token
**deleteOneToken**](token_api.md#deleteOneToken) | **DELETE** /token/{UserId}/{Tokenstr} | Deletes a specific token
**deleteOneUser**](token_api.md#deleteOneUser) | **DELETE** /token/{UserId} | Deletes the user and all its tokens
**getAccessRights**](token_api.md#getAccessRights) | **GET** /token/{UserId}/{Tokenstr}/access_right | Returns a list of access rights for a specific token
**getAllTokens**](token_api.md#getAllTokens) | **GET** /token | Returns a list of all existing tokens
**getOneAccessRight**](token_api.md#getOneAccessRight) | **GET** /token/{UserId}/{Tokenstr}/access_right/{AccessRightPath} | Returns information about a specific access right for a specific token
**getUserTokens**](token_api.md#getUserTokens) | **GET** /token/{UserId} | Returns all tokens of the user
**postAccessRight**](token_api.md#postAccessRight) | **POST** /token/{UserId}/{Tokenstr}/access_right | Creates a new access right for a specific token
**postToken**](token_api.md#postToken) | **POST** /token | Creates a new token
**postUserToken**](token_api.md#postUserToken) | **POST** /token/{UserId} | Creates a new token of the given `tokenstr`


# **deleteOneAccessRight**
> deleteOneAccessRight(ctx, ctx, user_id, tokenstr, access_right_path)
Deletes a specific access right for a specific token

Deletes a specific access right for a specific token. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_id** | **i32**| The numeric ID of the user | 
  **tokenstr** | **String**| The token string of the token to retrieve | 
  **access_right_path** | **String**| The path of the access right | 

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteOneToken**
> deleteOneToken(ctx, ctx, user_id, tokenstr)
Deletes a specific token

Deletes a specific token of the given token string and user ID. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_id** | **i32**| The numeric ID of the user | 
  **tokenstr** | **String**| The token string of the token to retrieve | 

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteOneUser**
> Vec<models::Token> deleteOneUser(ctx, ctx, user_id)
Deletes the user and all its tokens

Deletes the user and all its tokens If the operation was successful (status code `200`), all deleted tokens will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_id** | **i32**| The numeric ID of the user | 

### Return type

[**Vec<models::Token>**](Token.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAccessRights**
> Vec<models::AccessRight> getAccessRights(ctx, ctx, user_id, tokenstr, optional)
Returns a list of access rights for a specific token

Returns a list of access rights for a specific token. If the operation was successful (status code `200`), an array with access rights will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_id** | **i32**| The numeric ID of the user | 
  **tokenstr** | **String**| The token string of the token to retrieve | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user_id** | **i32**| The numeric ID of the user | 
 **tokenstr** | **String**| The token string of the token to retrieve | 
 **offset** | **i32**| The number of items to skip before starting to collect the result set | [default to 0]
 **limit** | **i32**| The number of items to return | [default to 100]

### Return type

[**Vec<models::AccessRight>**](AccessRight.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllTokens**
> Vec<models::Token> getAllTokens(ctx, ctx, optional)
Returns a list of all existing tokens

Returns a list of (all) existing tokens. With the optional parameter `TokenStrContains`, only the tokens with names containing all specified keywords (case insensitive) will be returned. If the operation was successful (status code `200`), an array with tokens will be returned. 

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
 **tokenstr_contains** | [**String**](String.md)| A set of keywords used to filter tokens. Only tokens that contain all of the specified keywords (case insensitive) will be returned. | 
 **offset** | **i32**| The number of items to skip before starting to collect the result set | [default to 0]
 **limit** | **i32**| The number of items to return | [default to 100]

### Return type

[**Vec<models::Token>**](Token.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOneAccessRight**
> models::AccessRight getOneAccessRight(ctx, ctx, user_id, tokenstr, access_right_path)
Returns information about a specific access right for a specific token

Returns information about a specific access right for a specific token. If the operation was successful (status code `200`), the access right will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_id** | **i32**| The numeric ID of the user | 
  **tokenstr** | **String**| The token string of the token to retrieve | 
  **access_right_path** | **String**| The path of the access right | 

### Return type

[**models::AccessRight**](AccessRight.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getUserTokens**
> Vec<models::Token> getUserTokens(ctx, ctx, user_id, optional)
Returns all tokens of the user

Returns a list of (all) existing tokens of the user. With the optional parameter `TokenStrContains`, only the tokens with names containing all specified keywords (case insensitive) will be returned. If the operation was successful (status code `200`), an array with tokens will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_id** | **i32**| The numeric ID of the user | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user_id** | **i32**| The numeric ID of the user | 
 **tokenstr_contains** | [**String**](String.md)| A set of keywords used to filter tokens. Only tokens that contain all of the specified keywords (case insensitive) will be returned. | 
 **offset** | **i32**| The number of items to skip before starting to collect the result set | [default to 0]
 **limit** | **i32**| The number of items to return | [default to 100]

### Return type

[**Vec<models::Token>**](Token.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postAccessRight**
> models::AccessRight postAccessRight(ctx, ctx, user_id, tokenstr, access_right)
Creates a new access right for a specific token

Creates a new access right for a specific token. If an access right with the same path already exists, it will be replaced by the new one.  If the operation was successful (status code `201`), all access rights of this token will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_id** | **i32**| The numeric ID of the user | 
  **tokenstr** | **String**| The token string of the token to retrieve | 
  **access_right** | [**AccessRight**](AccessRight.md)| The access right to create | 

### Return type

[**models::AccessRight**](AccessRight.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postToken**
> models::TokenId postToken(ctx, ctx, token)
Creates a new token

Creates a new token with no access rights. If the operation was successful (status code `201`), the ID of the created token will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | [**Token**](Token.md)| The token to create | 

### Return type

[**models::TokenId**](TokenId.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postUserToken**
> models::TokenId postUserToken(ctx, ctx, user_id, user_token)
Creates a new token of the given `tokenstr`

Creates a new token with no access rights. If the operation was successful (status code `201`), the ID of the created token will be returned. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_id** | **i32**| The numeric ID of the user | 
  **user_token** | [**UserToken**](UserToken.md)| The token string | 

### Return type

[**models::TokenId**](TokenId.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [ApiKeyAuthHeader](../README.md#ApiKeyAuthHeader)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

