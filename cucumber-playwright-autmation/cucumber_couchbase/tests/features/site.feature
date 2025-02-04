Feature: Rust Book Search with DataTable

  Scenario: Search for multiple books at once
    Given I am on the Rust book search page
    When I enter the following book titles in the "search" search field and click the "Search" button:
      | Book Title          |
      | ownership           |
      | concurrency         |
      | async programming   |
    Then I should see the books with the following titles in the results:
      | Book Title          |
      | ownership           |
      | concurrency         |
      | async programming   |
