Feature: Search Rust Books with Keywords

  Scenario: Search for Rust books by keyword "Concurrency"
    Given I am on the Rust Book search page
    When I search for "Understanding Ownership"
    Then I should see Rust books with "Concurrency" in the title or description
