namespace DemoBeCsas.Auth;

public static class AuthorizationExtensions
{
    public static IServiceCollection AddDemoBeAuthorization(this IServiceCollection services)
    {
        services.AddAuthorizationBuilder()
            .AddPolicy(
                "Admin",
                policy => policy.RequireAssertion(ctx =>
                    ctx.User.HasClaim("role", "ADMIN")
                )
            );
        return services;
    }
}
