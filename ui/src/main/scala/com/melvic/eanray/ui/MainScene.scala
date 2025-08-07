package com.melvic.eanray.ui

import scalafx.application.Platform
import scalafx.geometry.Orientation
import scalafx.scene.control.*
import scalafx.scene.layout.{BorderPane, StackPane}
import scalafx.scene.{Node, Scene}

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
            dividerPositions = 0.25
          }

          val leftSplitPane: SplitPane = new SplitPane:
            orientation = Orientation.Vertical
            items ++= Seq(new FitScrollPane(new CameraPane), new AvailableObjectsPane)

          val centerPane: StackPane = new StackPane {
            self =>
            content = new Viewport(self, 200, 100)
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
