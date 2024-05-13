# serializers.py
from rest_framework import serializers

class PaymentSerializer(serializers.Serializer):
    webhook_url = serializers.CharField()
    value = serializers.FloatField()

class ErrorSerializer(serializers.Serializer):
    message = serializers.CharField()
    error = serializers.CharField()