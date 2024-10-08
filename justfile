install-llama-models:
  sudo docker run -v .data/llama/models:/models ghcr.io/ggerganov/llama.cpp:full --all-in-one "/models/" 7B

test-llama-model:
  sudo docker run -v .data/llama/models:/models ghcr.io/ggerganov/llama.cpp:full --run -m /models/7B/ggml-model-q4_0.gguf -p "Building a website can be done in 10 simple steps:" -n 512
