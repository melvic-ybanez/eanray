package com.melvic.eanray.ui

import scalafx.geometry.Insets
import scalafx.scene.control.{Label, TextField, TitledPane}
import scalafx.scene.layout.GridPane

class CameraPane extends TitledPane:
  text = "Camera"
  collapsible = true
  maxHeight = Double.MaxValue

  content = new GridPane:
    margin = Insets(0, 0, 0, 10)
    hgap = 10
    vgap = 10

    def dimField = new TextField:
      prefColumnCount = 5
    addRow(0, Label("Dimensions"), dimField, dimField)

    val labels: Seq[String] = List(
      "Aspect Ratio",
      "Samples per pixel",
      "Max depth",
      "Field of view",
      "Look-from",
      "Look-at",
      "Vertical-up"
    )

    labels.zipWithIndex.foreach: (label, i) =>
      addColumn(0, Label(s"$label:"))
      add(TextField(), 1, i + 1, 2, 1)
