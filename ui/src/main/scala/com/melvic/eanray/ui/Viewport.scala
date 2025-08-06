package com.melvic.eanray.ui

import scalafx.scene.SceneAntialiasing.Balanced
import scalafx.scene.layout.Region
import scalafx.scene.{Node, SubScene}

class Viewport(container: Region, w: Double, h: Double) extends SubScene(w, h, true, Balanced):
  width <== container.widthProperty
  height <== container.heightProperty