interface config {
  with: withConfig;
  servers: server[];
}

interface server {
  addr: string;
  token: string[];
  passwd: string[];
}
