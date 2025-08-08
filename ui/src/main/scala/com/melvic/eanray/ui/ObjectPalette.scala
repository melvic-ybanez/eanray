package com.melvic.eanray.ui

import scalafx.scene.control.TitledPane

class ObjectPalette extends TitledPane:
  text = "Object Palette"
  collapsible = true
  maxHeight = Double.MaxValue

  content = new ObjectListPane