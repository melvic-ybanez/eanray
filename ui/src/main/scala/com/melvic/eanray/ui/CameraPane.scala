package com.melvic.eanray.ui

import com.melvic.eanray.ui.CameraPane.buildTextField
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

    def dimField: TextField =
      val field = buildTextField
      field.prefColumnCount = 4
      field

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
      add(buildTextField, 1, i + 1, 2, 1)

object CameraPane:
  def buildTextField: TextField =
    new TextField:
      prefColumnCount = 8

      style = """
          |-fx-background-color: #1e1e1e;
          |-fx-text-fill: white;
          |-fx-border-color: #3a3a3a;
      """.stripMargin
