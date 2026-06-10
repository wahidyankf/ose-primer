using DemoBeCsas.Auth;
using FluentAssertions;
using Microsoft.Extensions.Configuration;
using Xunit;

namespace DemoBeCsas.Tests.Unit;

/// <summary>
/// Tests for JwtService covering token creation, decoding, and exception paths.
/// </summary>
[Trait("Category", "Unit")]
public class JwtServiceTests
{
    private const string TestSecret = "test-jwt-secret-at-least-32-chars-long!!";

    private static JwtService CreateService()
    {
        var config = new ConfigurationBuilder()
            .AddInMemoryCollection(new Dictionary<string, string?> { ["CRUD_BE_CSHARP_ASPNETCORE_JWT_SECRET"] = TestSecret })
            .Build();
        return new JwtService(config);
    }

    [Fact]
    public void Constructor_ReadsSecretFromEnvironmentVariable()
    {
        // Arrange — wire the config key from the new canonical env var name
        const string envSecret = "env-driven-secret-for-unit-test-32chars!";
        var config = new ConfigurationBuilder()
            .AddInMemoryCollection(new Dictionary<string, string?>
            {
                ["CRUD_BE_CSHARP_ASPNETCORE_JWT_SECRET"] = envSecret,
            })
            .Build();

        // Act — construction must succeed (throws when key is missing)
        var service = new JwtService(config);

        // Assert — a token can be produced using that secret
        var token = service.CreateAccessToken(Guid.NewGuid().ToString(), "bob", "User");
        token.Should().NotBeNullOrEmpty();

        // And the token can be decoded with the same service (same secret was used)
        var principal = service.DecodeToken(token);
        principal.Should().NotBeNull();
    }

    [Fact]
    public void Constructor_ThrowsWhenJwtSecretKeyIsMissing()
    {
        // Arrange — configuration with no JWT secret key present
        var config = new ConfigurationBuilder()
            .AddInMemoryCollection(new Dictionary<string, string?>())
            .Build();

        // Act & Assert
        var act = () => new JwtService(config);
        act.Should().Throw<InvalidOperationException>()
            .WithMessage("*CRUD_BE_CSHARP_ASPNETCORE_JWT_SECRET*");
    }

    [Fact]
    public void CreateAccessToken_ReturnsNonEmptyToken()
    {
        var service = CreateService();
        var token = service.CreateAccessToken(Guid.NewGuid().ToString(), "alice", "User");
        token.Should().NotBeNullOrEmpty();
    }

    [Fact]
    public void CreateRefreshToken_ReturnsNonEmptyToken()
    {
        var service = CreateService();
        var token = service.CreateRefreshToken(Guid.NewGuid().ToString());
        token.Should().NotBeNullOrEmpty();
    }

    [Fact]
    public void DecodeToken_ValidToken_ReturnsPrincipal()
    {
        var service = CreateService();
        var token = service.CreateAccessToken(Guid.NewGuid().ToString(), "alice", "User");
        var principal = service.DecodeToken(token);
        principal.Should().NotBeNull();
    }

    [Fact]
    public void DecodeToken_InvalidToken_ReturnsNull()
    {
        // Covers the catch path in DecodeToken (lines 87-88)
        var service = CreateService();
        var principal = service.DecodeToken("not-a-valid-jwt-token");
        principal.Should().BeNull();
    }

    [Fact]
    public void GetJti_ValidToken_ReturnsJti()
    {
        var service = CreateService();
        var token = service.CreateAccessToken(Guid.NewGuid().ToString(), "alice", "User");
        var jti = service.GetJti(token);
        jti.Should().NotBeNullOrEmpty();
    }

    [Fact]
    public void GetJti_InvalidToken_ReturnsNull()
    {
        // Covers the catch path in GetJti (lines 100-102)
        var service = CreateService();
        var jti = service.GetJti("not-a-valid-jwt");
        jti.Should().BeNull();
    }

    [Fact]
    public void GetIssuedAt_ValidToken_ReturnsTimestamp()
    {
        var service = CreateService();
        var token = service.CreateAccessToken(Guid.NewGuid().ToString(), "alice", "User");
        var issuedAt = service.GetIssuedAt(token);
        issuedAt.Should().NotBeNull();
    }

    [Fact]
    public void GetIssuedAt_InvalidToken_ReturnsNull()
    {
        // Covers the catch path in GetIssuedAt (lines 116-118)
        var service = CreateService();
        var issuedAt = service.GetIssuedAt("not-a-valid-jwt");
        issuedAt.Should().BeNull();
    }
}
