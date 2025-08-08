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

    def widthField: SmallInputField = new SmallInputField:
      promptText = "Width"

    def heightField: SmallInputField = new SmallInputField:
      promptText = "Height"

    addRow(0, Label("Dimensions:"), widthField, Label("\u00D7"), heightField)
    addRow(1, Label("Aspect Ratio:"), widthField, Label(":"), heightField)

    val labels: Seq[String] = List(
      "Samples per pixel",
      "Max depth",
      "Field of view",
      "Look-from",
      "Look-at",
      "Vertical-up"
    )

    labels.zipWithIndex.foreach: (label, i) =>
      addColumn(0, Label(s"$label:"))
      add(new InputField, 1, i + 2, 3, 1)

class InputField extends TextField:
  prefColumnCount = 8

  style = """
      |-fx-background-color: #1e1e1e;
      |-fx-text-fill: white;
      |-fx-border-color: #3a3a3a;
        """.stripMargin

class SmallInputField extends InputField:
  prefColumnCount = 4
