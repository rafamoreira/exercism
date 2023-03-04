// Package weather provides the current weather condition for a given location.
package weather

// CurrentCondition holds a string representing the current weather condition.
var CurrentCondition string
// CurrentLocation holds a string representing the current location.
var CurrentLocation string

// Forecast returns a string with the current weather condition for a given location.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
