from PIL import Image
import pytesseract
import cv2
from pytesseract import Output


# sudo apt install tesseract-ocr
# sudo apt install libtesseract-dev
# sudo apt-get install tesseract-ocr-jpn
# pip install opencv-python
class Reader:
    # print(pytesseract.get_languages(config=''))
    # img = Image.open(self.img)
    # vin_box = img.crop((550, 85, self.w-160, self.h - 870))
    # img.show() PB=
    # print(pytesseract.image_to_string(vin_box, lang='eng', config=config))
    def __init__(self):
        self.img = 'static/8e1e5ea3-28b1-494a-b901-8fd89bfde6c6.jpg'
        self.w = 1000
        self.h = 1000
        self.cut_proportions = {
            "bid": 1
        }
    def cut(self):
        pass
    def read(self):
        config = '--psm 6 --oem 3'
        path = 'static/cv2/crop.jpg'
        image = cv2.imread(self.img)

        crop = image[85:self.h-870, self.w-444:self.w-164]
        cv2.imwrite(path, crop)
        crop = cv2.imread(path)
        crop_h, crop_w, _ = crop.shape

        boxes = pytesseract.image_to_boxes(crop, config=config, lang='eng')
        for box in boxes.splitlines():
            box = box.split(' ')
            print(box)
            crop = cv2.rectangle(crop, (int(box[1]), crop_h - int(box[2])), (int(box[3]), crop_h - int(box[4])), (255, 0, 0), 1)
        print(boxes)
        cv2.imshow('img', crop)
        cv2.waitKey(0)

