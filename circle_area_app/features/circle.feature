Feature: Calculate the area of a circle

  Scenario: Valid radius input
    Given I open the calculator page
    When I enter radius "5" and press calculate
    Then I should see the area displayed as "78.54"

  Scenario: Invalid radius input
    Given I open the calculator page
    When I enter radius "-5" and press calculate
    Then I should see an error message