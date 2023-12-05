provider "aws" {
  region = "us-west-2"
}

data "template_file" "user_data" {
  template = "${file("setup.sh")}"
}

resource "aws_instance" "snek" {
  ami           = "ami-058a0afa5f1acc977"  # Replace with a valid AMI for your region
  instance_type = "t4g.small"
  key_name      = "" # Replace with your key name

  # Security Group to allow Port 8000
  vpc_security_group_ids = [aws_security_group.instance_sg.id, aws_security_group.ssh_sg.id]

  # User data script to install Rust, unzip, and run the Rust project
  user_data = "${data.template_file.user_data.rendered}"

  # Provisioner to copy a ZIP file
  provisioner "file" {
    source      = "deploy.zip"
    destination = "/tmp/deploy.zip"

    connection {
      type        = "ssh"
      user        = "ec2-user"
      private_key = file("") # Replace with your private key path
      host        = self.public_ip
    }
  }
}

# Security Group
resource "aws_security_group" "instance_sg" {
  name        = "allow_8000"
  description = "Allow TCP 8000 inbound traffic"

  ingress {
    from_port   = 8000
    to_port     = 8000
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_security_group" "ssh_sg" {
  name        = "allow_ssh"
  description = "Allow SSH inbound traffic from my IP"

  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["<IP>/32"] # Replace with your IP
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}
