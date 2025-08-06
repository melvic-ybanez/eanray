import sbt._

object Dependencies {
  lazy val ui = Seq(
    "org.scalafx" %% "scalafx" % "21.0.0-R32",
    "io.github.mkpaz" % "atlantafx-base" % "2.0.1",
  )
}