# Phase is an SLM controller

# Structure of the data in the program
For the gui, there are 3 levels:
	- pattern controller
	- pattern container
	- main gui

## The pattern controller 
This is a control for one individual pattern
It is instantiated by giving it the data:
PatternControllerData (contains the information to recreate the pattern)
id (tells it which id it has)
container_relm (the reference to the container's relm)

## The pattern container
This contains a set of patterns, which are displayed with a common centre and cropping
## Main GUI
This contains a notebook which holds all of the pattern containers
