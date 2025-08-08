package com.melvic.eanray.ui

import scalafx.Includes.*
import scalafx.animation.*
import scalafx.application.Platform
import scalafx.geometry.Orientation
import scalafx.scene.control.*
import scalafx.scene.layout.{BorderPane, StackPane}
import scalafx.scene.{Node, Scene}
import scalafx.util.Duration

class MainScene extends Scene:
  root = new BorderPane:
    top = new MenuBar
    center = new SplitPane:
      orientation = Orientation.Vertical

      Platform.runLater {
        dividerPositions = 0.8
      }

      items ++= Seq(
        new SplitPane:
          orientation = Orientation.Horizontal
          Platform.runLater {
            dividerPositions = 0.23
          }

          val leftSplitPane: SplitPane = new SplitPane {
            leftSplitPane =>
            orientation = Orientation.Vertical

            val cameraPane: CameraPane = new CameraPane { self =>
              self.expanded.onChange: (_, _, newValue) =>
                val divider = leftSplitPane.dividers(0)
                val timeline = new Timeline:
                  keyFrames = Seq(
                    at(Duration(0))(
                      Set(
                        divider.positionProperty() -> (if newValue then 0 else divider.getPosition)
                      )
                    ),
                    at(Duration(300))(
                      Set(divider.positionProperty() -> (if newValue then 0.5 else 0))
                    )
                  )

                timeline.play()
            }

            items ++= Seq(cameraPane, new AvailableObjectsPane).map(new FitScrollPane(_))
          }

          val centerPane: StackPane = new StackPane {
            self =>
            children ++= Seq(new Viewport(self, 200, 100), new Label("Drag an object here to start") {
              style = "-fx-font-size: 50; -fx-text-fill: gray"
            })
          }

          items ++= Seq(leftSplitPane, centerPane)
        ,
        new TitledPane:
          text = "Logs"
          content = TextArea()
          prefHeight = -1
          maxHeight = Double.MaxValue
      )

class FitScrollPane(initContent: Node) extends ScrollPane:
  content = initContent
  fitToWidth = true
  fitToHeight = true
