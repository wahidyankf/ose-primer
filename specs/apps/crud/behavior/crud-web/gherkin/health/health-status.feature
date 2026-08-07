Feature: Service Health Status

  As an operations engineer
  I want to see the health status of the backend service in the app
  So that I can confirm the frontend is connected to a healthy backend

  Background:
    Given the app is running

  @unit @e2e
  Scenario: Health indicator shows the service is UP
    When the user opens the app
    Then the health status indicator should display "UP"

  @unit @e2e
  Scenario: Health indicator does not expose component details to regular users
    When an unauthenticated user opens the app
    Then the health status indicator should display "UP"
    And no detailed component health information should be visible

  @unit
  Scenario: Landing page explains its reference role
    When a visitor opens the app
    Then the landing page should identify itself as a reusable reference application
    And it should explain that the example can be adapted for a team's product
