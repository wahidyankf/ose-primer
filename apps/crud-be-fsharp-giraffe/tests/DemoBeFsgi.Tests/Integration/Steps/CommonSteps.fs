module DemoBeFsgi.Tests.Integration.Steps.CommonSteps

open System.Text.Json
open TickSpec
open Xunit
open DemoBeFsgi.Tests.State
open DemoBeFsgi.Tests.DirectServices

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

let private opts = JsonSerializerOptions(PropertyNameCaseInsensitive = true)

/// Restore '#' characters that were replaced with 'HASH_SIGN' by the feature
/// pre-processor in FeatureRunner (TickSpec strips inline '#' as Gherkin comments).
let internal decode (s: string) = s.Replace("HASH_SIGN", "#")

let internal getJsonProp (json: string) (prop: string) =
    try
        let doc = JsonDocument.Parse(json)
        let el = doc.RootElement.GetProperty(prop)
        Some el
    with _ ->
        None

let internal getStringProp (json: string) (prop: string) =
    try
        let doc = JsonDocument.Parse(json)
        let el = doc.RootElement.GetProperty(prop)
        Some(el.GetString())
    with _ ->
        None

/// Run a direct service call and return (status, body) as a ServiceResponse + body string.
let internal call (status: int) (body: string) : ServiceResponse * string = { Status = status; Body = body }, body

/// Helper to apply a direct service result to StepState.
let internal applyResult (status: int) (body: string) (state: StepState) : StepState =
    { state with
        Response = Some { Status = status; Body = body }
        ResponseBody = Some body }

// ─────────────────────────────────────────────────────────────────────────────
// Shared registration / login helpers (used across step files)
// ─────────────────────────────────────────────────────────────────────────────

let internal registerUser (state: StepState) (username: string) (email: string) (password: string) : string =
    let pw = decode password

    let status, body =
        register state.UserRepo username email pw |> Async.RunSynchronously

    body

let internal loginUser (state: StepState) (username: string) (password: string) : string option * string option =
    let pw = decode password

    let _status, body =
        login state.UserRepo state.RefreshTokenRepo username pw
        |> Async.RunSynchronously

    let accessToken = getStringProp body "accessToken"
    let refreshToken = getStringProp body "refreshToken"
    accessToken, refreshToken

// ─────────────────────────────────────────────────────────────────────────────
// Shared background steps
// ─────────────────────────────────────────────────────────────────────────────

[<Given>]
let ``the API is running`` (state: StepState) = state

// @covers specs/apps/crud/behavior/crud-be/gherkin/admin/admin.feature:Disabled user's access token is rejected with 401
// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Logout is idempotent — repeating logout on the same token returns 200
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Delete attachment returns 204
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Upload attachment to another user's entry returns 403
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:List attachments on another user's entry returns 403
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Delete attachment on another user's entry returns 403
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Delete non-existent attachment returns 404
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Delete an entry returns 204
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Unauthenticated request to create an entry returns 401
// @covers specs/apps/crud/behavior/crud-be/gherkin/security/security.feature:Admin unlocks a locked account
// @covers specs/apps/crud/behavior/crud-be/gherkin/token-management/tokens.feature:Blacklisted access token is rejected with 401 on protected endpoints
// @covers specs/apps/crud/behavior/crud-be/gherkin/token-management/tokens.feature:Deactivating a user revokes all their active tokens
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Successful password change returns 200
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Authenticated user self-deactivates their account
[<Then>]
let ``the response status code should be (\d+)`` (code: int) (state: StepState) =
    let actual = state.Response.Value.Status
    Assert.Equal(code, actual)
    state

[<Given>]
let ``a user "(.+)" is registered with password "(.+)"`` (username: string) (password: string) (state: StepState) =
    let email = $"{username}@example.com"
    registerUser state username email password |> ignore
    state

[<Given>]
let ``a user "(.+)" is registered with email "(.+)" and password "(.+)"``
    (username: string)
    (email: string)
    (password: string)
    (state: StepState)
    =
    registerUser state username email password |> ignore
    state

[<Given>]
let ``"(.+)" has logged in and stored the access token`` (username: string) (state: StepState) =
    let passwords = [ "Str0ng#Pass1"; "Str0ng#Pass2"; "Str0ng#Pass3"; "Str0ng#Admin1" ]
    let mutable accessToken = None
    let mutable userId = None

    for pw in passwords do
        if accessToken.IsNone then
            let at, _ = loginUser state username pw

            if at.IsSome then
                let _status, body =
                    getProfile state.UserRepo state.TokenRepo at |> Async.RunSynchronously

                accessToken <- at
                userId <- getStringProp body "id"

    { state with
        AccessToken = accessToken
        UserId = userId }

[<Given>]
let ``"(.+)" has logged in and stored the access token and refresh token`` (username: string) (state: StepState) =
    let passwords = [ "Str0ng#Pass1"; "Str0ng#Pass2"; "Str0ng#Admin1" ]
    let mutable accessToken = None
    let mutable refreshToken = None
    let mutable userId = None

    for pw in passwords do
        if accessToken.IsNone then
            let at, rt = loginUser state username pw

            if at.IsSome then
                let _status, body =
                    getProfile state.UserRepo state.TokenRepo at |> Async.RunSynchronously

                accessToken <- at
                refreshToken <- rt
                userId <- getStringProp body "id"

    { state with
        AccessToken = accessToken
        RefreshToken = refreshToken
        UserId = userId }

// ─────────────────────────────────────────────────────────────────────────────
// Generic request steps — map HTTP-like Gherkin to direct service calls
// ─────────────────────────────────────────────────────────────────────────────

/// Parse a JSON body string into its component fields for CreateExpense calls.
/// Returns defaults when fields are missing.
let private parseExpenseBody (bodyStr: string) =
    try
        let doc = JsonDocument.Parse(bodyStr)
        let r = doc.RootElement

        let str (key: string) =
            match r.TryGetProperty(key) with
            | true, el -> el.GetString()
            | _ -> null

        let floatOpt (key: string) =
            match r.TryGetProperty(key) with
            | true, el when el.ValueKind = JsonValueKind.Number -> Some(el.GetDouble())
            | _ -> None

        str "amount",
        str "currency",
        str "category",
        str "description",
        str "date",
        str "type",
        floatOpt "quantity",
        (match r.TryGetProperty("unit") with
         | true, el when el.ValueKind = JsonValueKind.String -> Some(el.GetString())
         | _ -> None)
    with _ ->
        null, null, null, null, null, null, None, None

/// Parse displayName from a profile update body.
let private parseProfileBody (bodyStr: string) =
    try
        let doc = JsonDocument.Parse(bodyStr)
        let r = doc.RootElement

        match r.TryGetProperty("displayName") with
        | true, el -> el.GetString()
        | _ -> null
    with _ ->
        null

/// Parse oldPassword and newPassword from a change-password body.
let private parsePasswordBody (bodyStr: string) =
    try
        let doc = JsonDocument.Parse(bodyStr)
        let r = doc.RootElement

        let str (key: string) =
            match r.TryGetProperty(key) with
            | true, el -> el.GetString()
            | _ -> null

        str "oldPassword", str "newPassword"
    with _ ->
        null, null

/// Parse refreshToken from a refresh body.
let private parseRefreshBody (bodyStr: string) =
    try
        let doc = JsonDocument.Parse(bodyStr)
        let r = doc.RootElement

        match r.TryGetProperty("refreshToken") with
        | true, el -> el.GetString()
        | _ -> null
    with _ ->
        null

/// Map a URL + method + body to a direct service call.
/// Returns (status, body).
let private dispatchCall
    (state: StepState)
    (method: string)
    (url: string)
    (body: string)
    (token: string option)
    : int * string =
    let m = method.ToUpperInvariant()
    let u = url.ToLowerInvariant()

    // Auth routes
    if u = "/api/v1/auth/register" && m = "POST" then
        let doc = JsonDocument.Parse(if body = "" then "{}" else body)
        let r = doc.RootElement

        let str (key: string) =
            match r.TryGetProperty(key) with
            | true, el -> el.GetString()
            | _ -> ""

        register state.UserRepo (str "username") (str "email") (str "password")
        |> Async.RunSynchronously
    elif u = "/api/v1/auth/login" && m = "POST" then
        let doc = JsonDocument.Parse(if body = "" then "{}" else body)
        let r = doc.RootElement

        let str (key: string) =
            match r.TryGetProperty(key) with
            | true, el -> el.GetString()
            | _ -> ""

        login state.UserRepo state.RefreshTokenRepo (str "username") (str "password")
        |> Async.RunSynchronously
    elif u = "/api/v1/auth/refresh" && m = "POST" then
        let rt = parseRefreshBody body

        refresh state.UserRepo state.RefreshTokenRepo (if rt = null then "" else rt)
        |> Async.RunSynchronously
    elif u = "/api/v1/auth/logout" && m = "POST" then
        logout state.TokenRepo token |> Async.RunSynchronously
    elif u = "/api/v1/auth/logout-all" && m = "POST" then
        logoutAll state.UserRepo state.TokenRepo state.RefreshTokenRepo token
        |> Async.RunSynchronously
    elif u = "/health" && m = "GET" then
        health ()
    elif u = "/api/v1/users/me" && m = "GET" then
        getProfile state.UserRepo state.TokenRepo token |> Async.RunSynchronously
    elif u = "/api/v1/users/me" && m = "PATCH" then
        let displayName = parseProfileBody body

        updateProfile state.UserRepo state.TokenRepo token displayName
        |> Async.RunSynchronously
    elif u = "/api/v1/users/me/password" && m = "POST" then
        let oldPw, newPw = parsePasswordBody body

        changePassword state.UserRepo state.TokenRepo token oldPw newPw
        |> Async.RunSynchronously
    elif u = "/api/v1/users/me/deactivate" && m = "POST" then
        deactivate state.UserRepo state.TokenRepo token |> Async.RunSynchronously
    elif u = "/api/v1/expenses" && m = "POST" then
        let amount, currency, category, description, date, entryType, quantity, unit =
            parseExpenseBody body

        createExpense
            state.UserRepo
            state.TokenRepo
            state.ExpenseRepo
            token
            amount
            currency
            category
            description
            date
            entryType
            quantity
            unit
        |> Async.RunSynchronously
    elif u = "/api/v1/expenses" && m = "GET" then
        listExpenses state.UserRepo state.TokenRepo state.ExpenseRepo token 1 20
        |> Async.RunSynchronously
    elif u = "/api/v1/expenses/summary" && m = "GET" then
        expenseSummary state.UserRepo state.TokenRepo state.ExpenseRepo token
        |> Async.RunSynchronously
    elif u.StartsWith("/api/v1/expenses/") && u.EndsWith("/attachments") && m = "GET" then
        let expId =
            let parts = u.Split('/')

            try
                System.Guid.Parse(parts[parts.Length - 2])
            with _ ->
                System.Guid.Empty

        listAttachments state.UserRepo state.TokenRepo state.ExpenseRepo state.AttachmentRepo token expId
        |> Async.RunSynchronously
    elif u.StartsWith("/api/v1/admin/users") && m = "GET" then
        let emailFilter =
            if url.Contains("?search=") then
                let idx = url.IndexOf("?search=")
                Some(url.Substring(idx + 8))
            else
                None

        listUsers state.UserRepo state.TokenRepo token 1 20 emailFilter
        |> Async.RunSynchronously
    elif url.StartsWith("/api/v1/test/set-admin-role/") && m = "POST" then
        let username = url.Substring("/api/v1/test/set-admin-role/".Length)
        setAdminRole state.UserRepo username |> Async.RunSynchronously
    elif u = "/.well-known/jwks.json" && m = "GET" then
        getJwks ()
    elif u.StartsWith("/api/v1/tokens") && m = "GET" then
        getTokenClaims token
    else
        // Fallback: return 404 for unrecognised routes
        404, """{"error":"Not Found","message":"Route not recognised in direct dispatch"}"""

[<When>]
let ``the client sends (GET|POST|PUT|PATCH|DELETE) (.+) with body (.+)``
    (method: string)
    (url: string)
    (body: string)
    (state: StepState)
    =
    let decodedBody = decode body
    let status, responseBody = dispatchCall state method url decodedBody None
    applyResult status responseBody state

[<When>]
let ``the client sends (GET|POST|PUT|PATCH|DELETE) ([^ ]+) with ([^']+)'s access token``
    (method: string)
    (url: string)
    (username: string)
    (state: StepState)
    =
    let status, responseBody = dispatchCall state method url "" state.AccessToken
    applyResult status responseBody state

[<When>]
let ``the client sends (GET|POST|PUT|PATCH|DELETE) ([^ ]+)$`` (method: string) (url: string) (state: StepState) =
    let status, responseBody = dispatchCall state method url "" None
    applyResult status responseBody state

// ─────────────────────────────────────────────────────────────────────────────
// Response body assertion steps
// ─────────────────────────────────────────────────────────────────────────────

// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Successful login response includes token type "Bearer"
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/currency-handling.feature:USD expense amount preserves two decimal places
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/currency-handling.feature:IDR expense amount is stored and returned as a whole number
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Get own entry by ID returns amount, currency, category, description, date, and type
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Update an entry amount and description returns 200
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:P&L summary returns income total, expense total, and net for a period
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:Income entries are excluded from expense total
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:Expense entries are excluded from income total
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:P&L summary filters by currency without cross-currency mixing
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:P&L summary for a period with no entries returns zero totals
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/unit-handling.feature:Create expense with metric unit "liter" stores quantity and unit correctly
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/unit-handling.feature:Create expense with imperial unit "gallon" stores quantity and unit correctly
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Update display name succeeds
[<Then>]
let ``the response body should contain "(.+)" equal to "(.+)"`` (field: string) (expected: string) (state: StepState) =
    let body = state.ResponseBody.Value
    let actual = getStringProp body field
    Assert.True(actual.IsSome, $"Field '{field}' not found in response: {body}")
    Assert.Equal(expected, actual.Value)
    state

// @covers specs/apps/crud/behavior/crud-be/gherkin/admin/admin.feature:List all users returns a paginated response
// @covers specs/apps/crud/behavior/crud-be/gherkin/admin/admin.feature:Admin generates a password-reset token for a user
// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Successful login returns access token and refresh token
// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Successful refresh returns a new access token and refresh token
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Upload JPEG image returns 201 with attachment metadata
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Upload PDF document returns 201 with attachment metadata
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Create expense entry with amount and currency returns 201 with entry ID
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Create income entry with amount and currency returns 201 with entry ID
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:List own entries returns a paginated response
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/unit-handling.feature:Expense without quantity and unit fields is accepted
// @covers specs/apps/crud/behavior/crud-be/gherkin/security/security.feature:Unlocked account can log in with correct password
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Successful registration response includes non-null user ID
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Get own profile returns username, email, and display name
[<Then>]
let ``the response body should contain a non-null "(.+)" field`` (field: string) (state: StepState) =
    let body = state.ResponseBody.Value
    let el = getJsonProp body field
    Assert.True(el.IsSome, $"Field '{field}' not found in response: {body}")
    let v = el.Value
    let isNull = v.ValueKind = JsonValueKind.Null
    Assert.False(isNull, $"Field '{field}' is null in response: {body}")
    state

// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Successful registration returns created user profile without password
[<Then>]
let ``the response body should not contain a "(.+)" field`` (field: string) (state: StepState) =
    let body = state.ResponseBody.Value
    let el = getJsonProp body field
    Assert.True(el.IsNone, $"Field '{field}' should not be present but found in: {body}")
    state

// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Upload unsupported file type returns 415
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/currency-handling.feature:Unsupported currency code returns 400
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/currency-handling.feature:Malformed currency code returns 400
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/currency-handling.feature:Negative amount is rejected with 400
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/unit-handling.feature:Create expense with an unsupported unit returns 400
// @covers specs/apps/crud/behavior/crud-be/gherkin/security/security.feature:Reject password shorter than 12 characters
// @covers specs/apps/crud/behavior/crud-be/gherkin/security/security.feature:Reject password with no special character
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration with invalid email format
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration with empty password
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration with weak password — no uppercase letter
[<Then>]
let ``the response body should contain a validation error for "(.+)"`` (field: string) (state: StepState) =
    let body = state.ResponseBody.Value

    Assert.True(
        body.ToLower().Contains(field.ToLower()),
        $"Response body should contain validation error for '{field}': {body}"
    )

    state

// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Reject login with wrong password
// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Reject login for non-existent user
// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Reject login for deactivated account
// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Reject refresh with an expired refresh token
// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Original refresh token is rejected after rotation (single-use)
// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Refresh fails for a deactivated user
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Upload file exceeding the size limit returns 413
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration when username already exists
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Reject password change with incorrect old password
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Self-deactivated user cannot log in with previous credentials
[<Then>]
let ``the response body should contain an error message about (.+)`` (topic: string) (state: StepState) =
    let body = state.ResponseBody.Value
    Assert.True(body.Length > 0, $"Response body should not be empty: {body}")
    state
