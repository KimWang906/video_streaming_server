import cv2
import os
import random

def extract_random_frame(video_path, subdir_name):
    vidcap = cv2.VideoCapture(video_path)
    frame_count = int(vidcap.get(cv2.CAP_PROP_FRAME_COUNT))
    frame_num = random.randint(0, frame_count - 1)
    vidcap.set(cv2.CAP_PROP_POS_FRAMES, frame_num)
    success,image = vidcap.read()
    if success:
        filename = f'{os.path.basename(video_path).split(".")[0]}.jpg'
        save_path = os.path.join('/home/kimwang09/Desktop/P.Files/video_streaming_server/src/images/', filename)
        cv2.imwrite(save_path, image)

root_dir = '/home/kimwang09/Desktop/P.Files/video_streaming_server/src/resources/my_hero_academia/'
for subdir in os.listdir(root_dir):
    subdir_path = os.path.join(root_dir, subdir)
    if os.path.isdir(subdir_path):
        for file in os.listdir(subdir_path):
            file_path = os.path.join(subdir_path, file)
            if file.endswith('.mp4'):
                extract_random_frame(file_path, subdir) # 랜덤한 프레임을 추출합니다.