package com.melvic.eanray.ui

import scalafx.scene.control.{Label, TextField, TitledPane}
import scalafx.scene.layout.GridPane

class CameraPane extends TitledPane:
  text = "Camera"
  collapsible = true
  maxHeight = Double.MaxValue

  content = new GridPane:
    hgap = 10
    vgap = 10

    val labels = List(
      "Width",
      "Height",
      "Samples per pixel",
      "Max depth",
      "Field of view",
      "Look-from",
      "Look-at",
      "Vertical-up"
    )
    labels.foreach: label =>
      addColumn(0, Label(s"$label:"))
      addColumn(1, TextField())
