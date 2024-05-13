import threading
import uuid

import requests
from rest_framework import status
from rest_framework.response import Response
from rest_framework.views import APIView

from .serializers import ErrorSerializer, PaymentSerializer


def approve_payment(serializer, payment_code):
    try:
        response = requests.post(
            url=serializer.data["webhook_url"],
            json={"payment_status": "success", "payment_code": str(payment_code)},
        )

        print(f"Response: {response}")
        response.raise_for_status()
    except Exception as e:
        error_serializer = ErrorSerializer({"message": "failed", "error": str(e)})


class PaymentView(APIView):
    def post(self, request):
        serializer = PaymentSerializer(data=request.data)

        try:
            if serializer.is_valid():
                serializer.data["value"]
                webhook_url = serializer.data["webhook_url"]
                payment_code = uuid.uuid4()
                print(f"Url do webhook:{webhook_url}")
                
                timer = threading.Timer(
                    5, approve_payment, args=(serializer, payment_code)
                )
                timer.start()

                print("Mock executado com sucesso")
                return Response({"message": "success", "payment_code": payment_code})
        except Exception as e:
            error_serializer = ErrorSerializer({"message": "failed", "error": str(e)})
            return Response(error_serializer.data, status=status.HTTP_400_BAD_REQUEST)
        return Response(status=status.HTTP_400_BAD_REQUEST)
