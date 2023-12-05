from huggingface_hub import InferenceClient


def main():
    chat = InferenceClient("bigscience/bloom-1b7")
    response = chat.text_generation("请给我的花店起个名")
    print(response)


if __name__ == "__main__":
    main()
