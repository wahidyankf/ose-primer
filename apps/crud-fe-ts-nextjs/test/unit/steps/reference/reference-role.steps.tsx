import path from "path";
import { loadFeature, describeFeature } from "@amiceli/vitest-cucumber";
import { render, screen, waitFor, cleanup } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { vi, expect } from "vitest";
import * as authApi from "@/lib/api/auth";
import HomePage from "@/app/page";

const feature = await loadFeature(
  path.resolve(
    __dirname,
    "../../../../../../specs/apps/crud/behavior/crud-web/gherkin/reference/reference-role.feature",
  ),
);

vi.mock("@/lib/api/auth", () => ({
  getHealth: vi.fn(),
  login: vi.fn(),
  register: vi.fn(),
  refreshToken: vi.fn(),
  logout: vi.fn(),
  logoutAll: vi.fn(),
}));

function createQueryClient() {
  return new QueryClient({
    defaultOptions: { queries: { retry: false }, mutations: { retry: false } },
  });
}

describeFeature(feature, ({ Scenario, Background }) => {
  Background(({ Given }) => {
    Given("the Next.js demo is running", () => {
      cleanup();
    });
  });

  Scenario("Landing page explains its reference role", ({ When, Then, And }) => {
    When("a visitor opens the Next.js demo", async () => {
      vi.mocked(authApi.getHealth).mockResolvedValue({ status: "UP" });
      render(
        <QueryClientProvider client={createQueryClient()}>
          <HomePage />
        </QueryClientProvider>,
      );
      await waitFor(() => {
        expect(screen.getByText("UP")).toBeInTheDocument();
      });
    });

    // @covers specs/apps/crud/behavior/crud-web/gherkin/reference/reference-role.feature:Landing page explains its reference role
    Then("the landing page should identify itself as a reusable reference application", () => {
      expect(screen.getByText(/reusable reference application/i)).toBeInTheDocument();
    });

    And("it should explain that the example can be adapted for a team's product", () => {
      expect(screen.getByText(/adapt.*team.*product/i)).toBeInTheDocument();
    });
  });
});
