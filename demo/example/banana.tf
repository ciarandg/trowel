resource "local_file" "banana" {
  content  = "bar!"
  filename = "${path.module}/banana.txt"
}
