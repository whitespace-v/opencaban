from NinjaDataBase import NinjaDataBase
from NinjaAuth import NinjaAuth
from NinjaCarPageParser import NinjaCarPageParser
from Ninja.NinjaCollector import NinjaCollector
from Reader import Reader
from chardet import detect
import unidecode
if __name__ == '__main__':
    ninjaAuth = NinjaAuth()
    ninjaAuth.auth()
    ninjaCollector = NinjaCollector(ninjaAuth)
    params = ninjaCollector.collect()
    ninjaCarPageParser = NinjaCarPageParser(ninjaAuth, params)
    cars = ninjaCarPageParser.run()
    NinjaDataBase.send(cars)
    # TODO: setup correct login
    # TODO: Обработать вылет сессии и старт парсера с того же места
    # TODO: multiprocessing

    # reader = Reader()
    # reader.read()

# https://www.araiaa.jp/en/news/group/



# AEP Nyusatsu
# ARAI Van&Truck
# Arai Kenki
# Arai Oyama
# Arai Oyama VT
# Arai Sendai
# AraiBaySide
# Aucnet
# BAYAUC
# CAA Chubu
# CAA Gifu
# CAA Tohoku
# CAA Tokyo
# HAA Kobe
# Hero AA
# Honda AA Hokkaido
# Honda AA Kansai
# Honda AA Kyushu
# Honda AA Nagoya
# Honda AA Sendai
# Honda AA Tokyo
# IAA Osaka
# ISUZU IMA Kobe
# Isuzu IMA Kyusyu
# Isuzu ima Tokyo
# JAA
# JU Aichi
# JU Akita
# JU Aomori
# JU Chiba
# JU Fukui
# JU Fukuoka
# JU Fukushima
# JU Gifu
# JU Gunma
# JU Hiroshima
# JU Hokkaido
# JU Ibaraki
# JU Ishikawa
# JU Kanagawa
# JU Kumamoto
# JU Mie
# JU Miyagi
# JU Miyazaki
# JU Nagano
# JU Nagasaki
# JU Nara
# JU Niigata
# JU Oita
# JU Okinawa
# JU Saitama
# JU Shimane
# JU Shizuoka
# JU Tochigi
# JU Tokyo
# JU Toyama
# JU Yamagata
# JU Yamaguchi
# JU Yamanashi
# KCAA Fukuoka
# KCAA Kyoto
# KCAA Minami Kyushu 2nd
# KCAA MinamiKyuusyu
# KCAA Yamaguchi
# LAA Okayama
# LAA Shikoku
# LUM Fukuoka
# LUM Hokkaido
# LUM Kobe
# LUM Nagoya
# LUM Tokyo
# MIRIVE Aichi
# MIRIVE Osaka
# Miraive Saitama
# NAA Fukuoka
# NAA Nagoya
# NAA Osaka
# NAA Tokyo
# NAA bid Club Nagoya
# Nissan Osaka AA
# Nissan Purazasoru bid Club Fukuoka
# Nissan Purazasoru bid Club Gifu
# Nissan Purazasoru bid Club Osaka
# Nissan Purazasoru bid Club Sendai
# Nissan Purazasoru bid Club Tochigi
# Nissan Purazasoru bid Club Tokyo
# Nissan Purazasoru bid Club Tomakomai
# ORIX bid Club Atsugi
# ORIX bid Club Fukuoka
# ORIX bid Club Kobe
# ORIX bid Club Sendai
# SAKURA Nyusatsu
# SOGO Hakata
# SOGO Internet
# SOGO Narita1
# SOGO Narita2
# SOGO Narita3
# SOGO Tomakomai
# Sapporo AA
# SuzukiAA Hamamatsu
# TAA Chubu
# TAA Hiroshima
# TAA Hokkaido
# TAA Hyogo
# TAA Kanto
# TAA Kinki
# TAA Kyushu
# TAA Minami Kyushu
# TAA Shikoku
# TAA Tohoku
# TAA Yokohama
# U Fukuoka
# U Gunma
# U Hokuriku
# U Kobe
# U Kyushu
# U Nagoya
# U Niigata
# U Okayama
# U Osaka
# U Saitama
# U Sapporo
# U Shizuoka
# U Tohoku
# U Tokyo
# U Yokohama
# USS-R Nagoya
# YANASE and AUCNET
# ZIP Osaka
# ZIP Tokyo
# ZeroCarsele Chiba
# ZeroCarsele Chubu
# ZeroCarsele Hakata
# ZeroCarsele Hokkaido
# ZeroCarsele Osaka
# ZeroCarsele Sendai
# ZeroCarsele Syonan