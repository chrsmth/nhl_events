# \StandingsApi

All URIs are relative to *https://statsapi.web.nhl.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_standing_types**](StandingsApi.md#get_standing_types) | **GET** /standingsTypes | Get all available NHL standing types.
[**get_standings**](StandingsApi.md#get_standings) | **GET** /standings | Get NHL division standings.
[**get_standings_by_type**](StandingsApi.md#get_standings_by_type) | **GET** /standings/{type} | Get NHL standings for a specific standing type.



## get_standing_types

> Vec<crate::models::StandingTypesInner> get_standing_types()
Get all available NHL standing types.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::StandingTypesInner>**](StandingTypes_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_standings

> crate::models::Standings get_standings(season, date)
Get NHL division standings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**season** | Option<**String**> | Standings for a specified season. |  |
**date** | Option<**String**> | Standings on a specified date. |  |

### Return type

[**crate::models::Standings**](Standings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_standings_by_type

> crate::models::Standings get_standings_by_type(r#type)
Get NHL standings for a specific standing type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | Standing types:   * `byConference` - Standings by Conference   * `byDivision` - Standings by Division   * `byLeague` - Standings by League   * `divisionLeaders` - Division Leader standings   * `postseason` - Postseason Standings   * `preseason` - Preseason Standings   * `regularSeason` - Regular Season Standings   * `wildCard` - Wild card standings   * `wildCardWithLeaders` - Wild card standings with Division Leaders  | [required] |

### Return type

[**crate::models::Standings**](Standings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

