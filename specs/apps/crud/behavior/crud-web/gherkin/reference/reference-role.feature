Feature: Next.js Reference Role

  As a product-minded engineer evaluating the starter
  I want the Next.js demo to explain its role
  So that I know I can adapt the example for my team's product

  Background:
    Given the Next.js demo is running

  @unit
  Scenario: Landing page explains its reference role
    When a visitor opens the Next.js demo
    Then the landing page should identify itself as a reusable reference application
    And it should explain that the example can be adapted for a team's product
