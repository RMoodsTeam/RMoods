import logging

logging.basicConfig(
    level=logging.DEBUG,
    format='%(levelname)s:     %(asctime)s - %(message)s',
    datefmt='%Y-%m-%d %H:%M:%S'
)

logger = logging.getLogger('nlp_service')
