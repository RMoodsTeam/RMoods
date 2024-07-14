import sys

sys.path.insert(1, "src")

from main import app
import json 


"""
Test function file, checking expected value of function and if they responses are correct.
"""


def test_get_json_data():
    """
    Test function for checking if data is correct in get_json_data function.
    We also check if we spot a wrong input correctly.
    """
    # Test for correct input
    response = app.test_client().get(
        '/json',
        data=json.dumps({'name': 'test', 'Modified': 'No'}),
        content_type='application/json',
    )

    data = json.loads(response.get_data(as_text=True))

    assert response.status_code == 200
    assert data['Modified'] == 'Yes'

    # Test for wrong input
    response = app.test_client().get(
        '/json',
        data="just text",
        content_type='text/plain',
    )
    
    assert response.status_code == 415
