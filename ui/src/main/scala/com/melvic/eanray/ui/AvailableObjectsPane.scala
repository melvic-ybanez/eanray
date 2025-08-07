package com.melvic.eanray.ui

import scalafx.scene.control.TitledPane

class AvailableObjectsPane extends TitledPane:
  text = "Objects"
  collapsible = true
  maxHeight = Double.MaxValue

  content = new ObjectListPane