provider "aws" {
  region = "us-east-1"
}

resource "aws_instance" "example" {
  ami           = "ami-12345678"  # Replace with a valid AMI for your region
  instance_type = "t2.micro"
  key_name      = "your-key-name" # Replace with your key name

  # Security Group to allow Port 8000
  vpc_security_group_ids = [aws_security_group.instance_sg.id]

  # User data script to install Rust, unzip, and run the Rust project
  user_data = <<-EOF
              #!/bin/bash
              # Install Rust
              curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
              source $HOME/.cargo/env

              # Install unzip
              sudo dnf install -y unzip

              # Unzip the file to the home directory
              mkdir $HOME/project
              unzip /tmp/deploy.zip -d $HOME/project

              # Change directory to the project and build the Rust project
              cd $HOME/project
              cargo build --release

              # Run the Rust project
              ./target/release/boa-checker-snek
              EOF

  # Provisioner to copy a ZIP file
  provisioner "file" {
    source      = "deploy.zip"
    destination = "/tmp/fildeploye.zip"
  }

  tags = {
    Name = "ExampleInstance"
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