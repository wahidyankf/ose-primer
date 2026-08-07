Feature: Demo Reference Role

  As a product-minded engineer evaluating a demo application
  I want the landing page to explain its role
  So that I know I can adapt the example for my team's product

  Background:
    Given the demo application is running

  @unit
  Scenario: Landing page explains its reference role
    When a reader opens the demo application
    Then the landing page should identify itself as a reusable reference application
    And it should explain that the example can be adapted for a team's product
