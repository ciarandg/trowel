resource "null_resource" "orange" {
  provisioner "local-exec" {
    command = "echo orange"
  }
}
