package com.melvic.eanray.ui

import scalafx.scene.control.TitledPane

class ObjectPalette extends TitledPane:
  text = "Available Objects"
  collapsible = true
  maxHeight = Double.MaxValue

  content = new ObjectListPane