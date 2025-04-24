resource "local_sensitive_file" "apple" {
  content  = "foo!"
  filename = "${path.module}/apple.txt"
}
