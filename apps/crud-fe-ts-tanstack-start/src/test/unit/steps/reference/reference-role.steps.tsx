import path from "path";
import React from "react";
import { loadFeature, describeFeature } from "@amiceli/vitest-cucumber";
import { render, screen, waitFor, cleanup } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { vi, expect } from "vitest";
import * as authApi from "@/lib/api/auth";
import { Route as HomeRoute } from "@/routes/index";

const feature = await loadFeature(
  path.resolve(
    __dirname,
    "../../../../../../../specs/apps/crud/behavior/crud-web/gherkin/reference/reference-role.feature",
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

vi.mock("@tanstack/react-router", () => ({
  createFileRoute: (_path: string) => (opts: { component: React.ComponentType }) => ({
    options: opts,
    component: opts.component,
  }),
  Link: ({ children, to, style }: { children: React.ReactNode; to: string; style?: React.CSSProperties }) => (
    <a href={to} style={style}>
      {children}
    </a>
  ),
}));

function createQueryClient() {
  return new QueryClient({
    defaultOptions: { queries: { retry: false }, mutations: { retry: false } },
  });
}

function HomePageWrapper() {
  const Component = HomeRoute.options.component as React.ComponentType;
  return <Component />;
}

describeFeature(feature, ({ Scenario, Background }) => {
  Background(({ Given }) => {
    Given("the demo application is running", () => {
      cleanup();
    });
  });

  Scenario("Landing page explains its reference role", ({ When, Then, And }) => {
    When("a reader opens the demo application", async () => {
      vi.mocked(authApi.getHealth).mockResolvedValue({ status: "UP" });
      render(
        <QueryClientProvider client={createQueryClient()}>
          <HomePageWrapper />
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
